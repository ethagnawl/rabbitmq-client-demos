## RabbitMQ C Client Demo

### What?
A proof-of-concept program which demonstrates how you could read messages from
a RabbitMQ message queue using the librabbitmq C library.

### Why?
A few years ago, I worked on [a (really fun) project](http://fakelove.tv/work/star-wars-premiere) whose front-end was
an [openFrameworks](http://openframeworks.cc/) application. Due to time/resource constraints, the
developers were unable to get the ofx application to pull messages directly
from the back-end service I'd built, which exposed a RabbitMQ message queue. We
ended up hacking together a Rube Goldberg solution which used a Ruby program to
read messages from the RabbitMQ queue and pass them along to the ofx application
using OSC -- which ended up working really well. (For anyone who may be
interested in reading more about the problems we faced and the solutions we came
up with, a case study is available on [my website](http://peterdohertys.website/case-studies/star-wars-red-carpet-sound-off.html).)

However, I've always wanted to try writing a C program to do this, and since
I've recently been brushing up on C, I thought now would be a good time to try.

### Go on ...
My demo relies _heavily_ on the [amqp_listen](https://github.com/alanxz/rabbitmq-c/blob/master/examples/amqp_listen.c) example provided by the
[rabbitmq-c project](https://github.com/alanxz/rabbitmq-c). The (marginally) interesting modifications include: use
of environment variables, client authentication and storing the contents of
message bodies for use in other parts of the program (e.g. in the program
mentioned above, the message body would be used to control which set of videos
was shown on the display wall). These are admittely all pretty trivial, but I'm
still learning and I feel pretty good about the program, my experience reading
through and making use of the types provided by the library, working with the C
toolchain, etc.

I'm ready to call this experiment a success, as I was able to
sucessfully resurrect the back-end for the project mentioned above and get the
demo authenticating against and pulling messages off the queue.
