# Buffers

When working with bytes, what we really mean is working with byte _buffers_. Your first
thought for a buffer to use would be a `Vec<u8>`. This is a good first choice in many applications,
but doesn't work well in network applications. When sending data, you might consume the first half of the data,
which requires re-copying all the bytes around the buffer. Instead, a `VecDeque<u8>` ring buffer tends to be more appropriate.

However, if you consider the example of HTTP, where we have a task per request, all sharing the same TCP stream in a separate task.
When streaming bytes out of your HTTP request, you will have to share those bytes to some other task. This ends requiring you
shuffle allocations between tasks constantly which ends up being very awkward, since we unfortunately do not
have a garbage collector to help us when sharing memory between tasks. To deal with this, there's a convenient
crate that tries to put reference counters (like Arc) in the right places so you can mostly ignore this.

Unsurprisingly, this crate is called [`bytes`](https://docs.rs/bytes/latest/bytes/). It is well integrated with tokio.

```
   Arc ptrs                   ┌─────────┐
   ________________________ / │ Bytes 2 │
  /                           └─────────┘
 /          ┌───────────┐     |         |
|_________/ │  Bytes 1  │     |         |
|           └───────────┘     |         |
|           |           | ___/ data     | tail
|      data |      tail |/              |
v           v           v               v
┌─────┬─────┬───────────┬───────────────┬─────┐
│ Arc │     │           │               │     │
└─────┴─────┴───────────┴───────────────┴─────┘
```

There's only 2 types in this crate. `Bytes` and `BytesMut`. As the names imply, one is read-only, the other is read-write.
Importantly, you can split a `BytesMut` into two, sharing the same buffer - and then you can freely convert
those `BytesMut`s into `Bytes`. And of course, these are `Send` so are safe to share between tasks.

## AsyncRead

One advantage of `tokio::io` over `std::io` is the ability to read into un-initialised memory.
This can be a nice optimisation as you don't need to run an "expensive" routine to write 0s into
the 1MB buffer (only 5us on my hardware, but still!).

To utilise this, we can use the `read_buf` method.

```rust
// with read, you have to pre-initialise the bytes
let mut buffer = vec![0; 1024*1024];

// read into our buffer
let n = stream.read(&mut buffer).await?;

// actually available data
let data = &buffer[..n];
```

```rust
// with read_buf, you don't even need to pre-allocate, but we can if we think it will improve efficiency.
let mut buffer = Vec::with_capacity(1024*1024);

// the buffer will be resized as data is read, and it is always read to the end of the buffer.
// no accidental over-writes!
stream.read_buf(&mut buffer).await?;

// all initialised data in the vec is available to read. no need to track the length.
let data = &buffer;
```

## AsyncWrite

AsyncWrite also has better buffer support.

In `std::io::Write`, you have `write()` and `write_all()`. These two are technically enough to do anything, but it could be annoying.

Let's say you want to wait until a request header is written, but then you are happy to let the remainder of the body be written
asynchronously. You have the header and some of the body written into a `Vec<u8>`. Using `write_all` here would be wrong,
since you will end up waiting until the entire buffer is sent, not just when the header is sent. So we need to use `write` to get
what we want.

```rust
// assuming these exist
let mut buf: Vec<u8>;
let mut stream: TcpStream;

let mut header_length: usize = HEADER_LEN;
let mut written: usize = 0;

while written < header_length {
    let n = stream.write(&buf[written..])?;
    written += n;
}

// remove it from our buffer.
buf.drain(..written);
```

Using a `Vec` here isn't ideal with the last step, as it needs to copy the bytes to the start of the vec. a `Bytes` would work better, so let's use that, along with tokio's `write_buf`.


```rust
// assuming these exist
let mut buf: Bytes;
let mut stream: TcpStream;

let mut header_length: usize = HEADER_LEN;
let mut written: usize = 0;

while written < header_length {
    let n = stream.write_buf(&mut buf).await?;
    written += n;
}

// the written data was already removed from our buffer.
```

While the changes are quite minor, it removes some places where subtle bugs can creep in.
It's really easy to forget to use `&buf[written..]` when chaining calls to `write`, I am guilty of writing code like this
and getting really broken data as a result.
