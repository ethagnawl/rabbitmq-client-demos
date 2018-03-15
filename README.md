## RabbitMQ C Client Demo

### What?
A proof-of-concept program which demonstrates how to read messages from a
RabbitMQ message queue using the librabbitmq C library.

### Why?
A few years ago, I worked on [a (very exciting) project](http://fakelove.tv/work/star-wars-premiere) whose front-end was
an [openFrameworks](http://openframeworks.cc/) application. Due to time/resource constraints, the
developers were unable to get the ofx application to pull messages directly
from the back-end's message queue. We ended up hacking together a Rube Goldberg
solution -- a Ruby program which read messages from RabbitMQ and passed them
to the ofx application via OSC -- which wound up being _good enough_. (For
anyone who may be interested in reading more about the problems we faced and
the solutions we came up with, a case study is available on [my website](http://peterdohertys.website/case-studies/star-wars-red-carpet-sound-off.html).)

That all being said, I've always wanted to revisit this project and try writing
a C program to pull messages from the RabbitMQ message queue. Since I've
recently been brushing up on C, I thought now would be a good time to try.

### Go on ...
My demo relies _heavily_ on the [amqp_listen](https://github.com/alanxz/rabbitmq-c/blob/master/examples/amqp_listen.c) example provided by the
[rabbitmq-c project](https://github.com/alanxz/rabbitmq-c). The (marginally) interesting modifications include: use
of environment variables, client authentication and storing the contents of
message bodies for use in other parts of the program (in the program mentioned
above, the message body would be used to determine which set of videos should be
shown on the display wall). These are admittely all pretty trivial, but I feel
good about the experience: the program itself, reading through the library's
source, working with the C toolchain, etc.

I'm ready to call this experiment a success. I was able to sucessfully
resurrect the project's back-end and get the demo authenticating against and
pulling messages off the queue.
