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
The root URI (e.g. localhost:8080/) will always serve index.html from the specified root directory.
The default configuration file is located at:

    ~/.config/easy-http/default-config.toml
and should look like this:

    bind_to = '127.0.0.1:8080' # What the server should bind to, e.g. 192.168.1.100:8080
    html_path = 'html' # The root directory from which HTML files should be served
    not_found_path = '404.html' # The path to the 404 HTML file
Change these options as needed.
