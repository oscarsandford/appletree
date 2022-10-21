# Appletree in Eden

Appletree is [Applebot](https://github.com/oscarsandford/applebot) upgraded with TypeScript, as well as its own local Rust web server and SQL database called Eden.

## Prerequisites
Install [`node`](https://nodejs.org/en/), [`npm`](https://www.npmjs.com/), and [Rust](https://www.rust-lang.org/).

## Build

See the instructions for building the database files under `eden/db`.

## Deploy Locally

Eventually and hopefully, Docker images will be created and hosted on GitHub that can be pulled from. However, you can clone this repository, navigate to the repository root with the `docker-compose.yml` file, and simply build and run the composition with 
```
docker compose up -d
```
view logs while running with
```
docker compose logs
```
and shut it down with 
```
docker compose down
```

## Deploy Remotely

If you want to deploy this project to a remote host, here's a quick tutorial that is also general purpose:
* Make sure you can SSH with password-less authentication (i.e. using a SSH key)
```sh
ssh-keygen -t rsa        # Do NOT enter a passphrase when prompted!
ssh-copy-id -i ~/.ssh/my_id.pub user@hostname
```
* Use Docker contexts to create a new context (e.g. "`remote_name`") with the remote host, and you can then use it to manage Docker on that machine.
```sh
docker context list
docker context create remote_name --docker "host=ssh//user@hostname"
docker --context remote_name ps                 # List containers on remote_name context.
docker-compose --context remote_name up -d      # For composing on remote_name context.
docker-compose --context remote_name down       # Shut down.
# In order to view logs, I found that the only way was to switch contexts first.
docker context use remote_name                  # Use this context.
docker compose logs                             # View logs for this project (make sure cwd is the repo root).
```
* ***Note*** - If you are getting errors about a Docker daemon not running while using the context, you might need to do this so it can run in user mode:
```sh
sudo groupadd docker
sudo usermod -aG docker $(whoami)
# Log out and then log back in to ensure docker runs with correct perms.
sudo service docker start
```


## Seed
In order to make use of slash commands, you must run the script to register them with Discord's API using the following npm command:
```sh
npm run seed
```
The propagation time may vary, but expect it to take a while, especially when running the global configuration. See `scripts/seed.js` for more info on how to do this.