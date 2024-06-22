{ pkgs, lib, config, inputs, ... }:
let
  # Use for older versions of NodeJS, needs input `nixpkgs-nodejs`
  # pkgsNodejs = import
  #   inputs.nixpkgs-nodejs
  #   { inherit system; };
  localDomain = "rustybarn.local";

  buildDeps = {
    inherit (pkgs) nodejs_20;
    # For older version of NodeJS: pkgsNodejs.nodejs-slim-8_x
  };

  devDeps = buildDeps // {
    # inherit (pkgs) rustup;
    inherit (pkgs) cargo-watch;
    inherit (pkgs) gh;
  };

  containerRuntimeDeps = {
    inherit (pkgs) nginx;
  };
in
{
  options.containerDeps = lib.mkOption {
    type = lib.types.package;
    description = "Dependencies for prod container.";
  };

  config = {
    name = "weland-wp";

    # outputs.devDeps = devDeps;
    # Override `.env`
    env = {
      # DB_HOST = "127.0.0.1";
      # NGINX_HOST = "localhost";
      # NGINX_PORT = 8081;
    };

    # languages = {
    #   rust = {
    #     enable = true;
    #     channel = "nixpkgs";
    #   };
    # };

    containerDeps = pkgs.buildEnv {
      name = "container-env";
      paths = builtins.attrValues containerRuntimeDeps;
    };

    # Include git
    packages = [ pkgs.git ] ++ builtins.attrValues devDeps;

    # scripts.build.exec = "./scripts/build.sh";
    scripts.rndport.exec = ''
      sed -i"" '/^NGINX_PORT/d' .env; echo NGINX_PORT=$((RANDOM % 29000 + 3000)) \
        >> .env
    '';
    # scripts.deploy.exec = "./scripts/deploy.sh";

    processes = {
      app.exec = "cargo watch -x run; ./target/release/rusty-barn";
    };

    certificates = [
      localDomain
    ];
    hosts."${localDomain}" = "127.0.0.1";

    services.nginx = {
      enable = lib.mkDefault true;
      httpConfig = lib.mkDefault ''
        server {
          listen ${toString config.env.NGINX_PORT};
          listen ${toString config.env.NGINX_SSL_PORT} ssl;
          ssl_certificate     ${config.env.DEVENV_STATE}/mkcert/${localDomain}.pem;
          ssl_certificate_key ${config.env.DEVENV_STATE}/mkcert/${localDomain}-key.pem;
          # ssl_protocols       TLSv1 TLSv1.1 TLSv1.2 TLSv1.3;
          # ssl_ciphers         HIGH:!aNULL:!MD5;

          root ${config.env.DEVENV_ROOT}/public;
          index index.php index.html index.htm;
          server_name ${config.env.NGINX_HOST};
          client_max_body_size 64m;

          proxy_read_timeout 300;

          error_page 497 https://$server_name:$server_port$request_uri;

          location / {
            proxy_pass http://127.0.0.1:5000;

            proxy_set_header Host $http_host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
          }
        }
      '';
    };

    # See full reference at https://devenv.sh/reference/options/
    dotenv.enable = true;
  };
}
