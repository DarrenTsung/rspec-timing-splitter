FROM osig/rust-ubuntu:1.31

WORKDIR /code

COPY . .
RUN cargo build --release

CMD /bin/bash
