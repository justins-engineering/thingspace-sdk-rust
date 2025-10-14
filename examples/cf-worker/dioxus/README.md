# Perch
A web client designed for an Non-IP NBIoT middleware server

## Development

### Requirements
- [Bun](https://bun.com/get)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started/)
- [Docker Compose](https://docs.docker.com/compose/install/)

### Tailwind CSS
1. Run the following command in the root of the project:
```bash
bun install
```
2. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
bunx @tailwindcss/cli -i ./assets/tailwind.css -o ./assets/styling/main.css --watch
```

### Starting Kratos

Run the following command in the root of your project:

```bash
docker-compose up --force-recreate
```

### Serving The App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --addr 127.0.0.1 --port 4455
```

- Open the web app at http://127.0.0.1:4455
- Open MailSlurper at http://127.0.0.1:4436
- Open [kratos-admin-ui](https://github.com/dhia-gharsallaoui/kratos-admin-ui) at http://127.0.0.1:3000
