MPF Crash Reporter Backend
==========================

Backend for the MPF Crash Reporter

Building
--------

   cargo build

or:

   docker build -t mpf-crash-reporter .


Running
-------

   cargo run

or:

   docker run -it --rm --name mpf-crash-reporter-instance mpf-crash-reporter

or:

   helm install helm-chart/mpf-crash-reporter
