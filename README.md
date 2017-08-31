![](experimental.svg)

This service is not ready for use yet. Please feel free to test it out in local sandboxed environments (e.g. a VM) but don't use it for anything live yet.

# Better Than Basic: Better than Basic Authentication

Better Than Basic is a simple authentication service for use with nginx's http auth request module. The goal of the service is to provide password protection for areas on your webservice with a better user experience than basic authentication – while not being harder or more complicated to set up and configure than the `auth_basic` directive.

## Installation

Packages coming soon. For now you need to build it yourself with Rust & Cargo.

## Configuration

### ngnix

```nginx
location /private {
    auth_request /login/check;
    error_page 403 =303 $scheme://$http_host/login/?return=$request_uri;
}

location /login/ {
    proxy_pass http://localhost:3000/;
}
```

### better-than-basic

The configuration file `config.toml` sets options for the Better Than Basic service itself:

```toml
listen = 'localhost:3000'
```

### users file

Just like basic authentication, the credentials are held in a flat file rather than a database. Passwords must be hashed with bcrypt:

```toml
guest = '$2a$10$jGz8blre33lHrfIza/j0X.0Y/qKRH3gN7xqrKFtyH/W.7rWlQtxqi'
```

## Advanced

### Listening on a Unix socket

You can define which Unix socket to listen on and what permissions to set on it:

```toml
listen = '/tmp/socket' # for UNIX socket
socket_mode = '777'
```

And then configure nginx to connect to that socket:

```nginx
location /login/ {
    proxy_pass http://unix:/tmp/socket:/;
    #proxy_redirect http://locahost:3000/ http://$http_host:2210/login/;
}
```

You will probably need to configure the proxy_redirect directive. I haven't figured that part out yet.
