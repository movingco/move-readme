initSidebarItems({"fn":[["copy","Asynchronously copies the entire contents of a reader into a writer."],["copy_bidirectional","Copies data in both directions between `a` and `b`."],["copy_buf","Asynchronously copies the entire contents of a reader into a writer."],["duplex","Create a new pair of `DuplexStream`s that act like a pair of connected sockets."],["empty","Creates a new empty async reader."],["repeat","Creates an instance of an async reader that infinitely repeats one byte."],["sink","Creates an instance of an async writer which will successfully consume all data."],["split","Splits a single value implementing `AsyncRead + AsyncWrite` into separate `AsyncRead` and `AsyncWrite` handles."],["stderr","Constructs a new handle to the standard error of the current process."],["stdin","Constructs a new handle to the standard input of the current process."],["stdout","Constructs a new handle to the standard output of the current process."]],"mod":[["unix","Asynchronous IO structures specific to Unix-like operating systems."]],"struct":[["BufReader","The `BufReader` struct adds buffering to any reader."],["BufStream","Wraps a type that is [`AsyncWrite`] and [`AsyncRead`], and buffers its input and output."],["BufWriter","Wraps a writer and buffers its output."],["DuplexStream","A bidirectional pipe to read and write bytes in memory."],["Empty","An async reader which is always at EOF."],["Interest","Readiness event interest."],["Lines","Reads lines from an `AsyncBufRead`."],["ReadBuf","A wrapper around a byte buffer that is incrementally filled and initialized."],["ReadHalf","The readable half of a value returned from `split`."],["Ready","Describes the readiness state of an I/O resources."],["Repeat","An async reader which yields one byte over and over and over and over and over and…"],["Sink","An async writer which will move data into the void."],["Split","Splitter for the `split` method."],["Stderr","A handle to the standard error stream of a process."],["Stdin","A handle to the standard input stream of a process."],["Stdout","A handle to the standard output stream of a process."],["Take","Stream for the `take` method."],["WriteHalf","The writable half of a value returned from `split`."]],"trait":[["AsyncBufRead","Reads bytes asynchronously."],["AsyncBufReadExt","An extension trait which adds utility methods to `AsyncBufRead` types."],["AsyncRead","Reads bytes from a source."],["AsyncReadExt","Reads bytes from a source."],["AsyncSeek","Seek bytes asynchronously."],["AsyncSeekExt","An extension trait that adds utility methods to `AsyncSeek` types."],["AsyncWrite","Writes bytes asynchronously."],["AsyncWriteExt","Writes bytes to a sink."]]});