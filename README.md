# easy-http
### An easy HTTP server for static HTML sites.

### Usage:
Clone the repository and build with:

    $ git clone https://github.com/CarterTheCoder/easy-http.git
    $ cargo build
to run:

    $ cd target
    $ cd release
    $ ./easy-http
### Config:
The default configuration file is located at:

    ~/.config/easy-http/default-config.toml
and should look like this:

    bind_to = '127.0.0.1:8080'
    html_path = 'index.html'
    not_found_path = '404.html'
Change these options as needed.

   

