# instapets

Social network for pets

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

create a database:
```sh
docker run --name pets -p 5432:5432 -e POSTGRES_PASSWORD=pets -e POSTGRES_USER=pets -e POSTGRES_DB=pets -d postgres
```

run migrations:
```sh
diesel migration run
```

set a env file with the db credentials
```sh
echo DATABASE_URL=postgres://pets:pets@localhost:5432/pets > .env
```

run

```sh
cargo run
```

you will see something similar to:

```sh
Mounting /api/v1/:
    => POST /api/v1/user application/json (new_user)
    => GET /api/v1/users/<username> application/json (get_user)
🚀 Rocket has launched from http://localhost:8000
```

### Prerequisites

you need rust and diesel

[follow the guide here](https://medium.com/better-programming/rest-api-in-rust-step-by-step-guide-b8a6c5fcbff0)

```
Give examples
```

### Installing

To install rust follow [this ](https://www.rust-lang.org/tools/install)

To install diesel follow [this](http://diesel.rs/guides/getting-started/)

## Running the tests

Explain how to run the automated tests for this system

## Deployment

Add additional notes about how to deploy this on a live system

## Built With

* [Dropwizard](http://www.dropwizard.io/1.0.2/docs/) - The web framework used
* [Maven](https://maven.apache.org/) - Dependency Management
* [ROME](https://rometools.github.io/rome/) - Used to generate RSS Feeds

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/your/project/tags). 
