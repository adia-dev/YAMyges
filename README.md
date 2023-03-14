# MyGES Calendar App

This is an application that displays the calendar for the courses of ESGI, a school based in France in Paris.
The platform used to access the calendar is Kordis.

The application is built with Rust for the backend and Elixir / Phoenix for the frontend.

## Features

- Display the calendar for the courses of ESGI
- Retrieve data from MyGES platform
- Filter courses by date, course name or teacher name (WIP)
- View detailed information for each course, including date, time, location, and teacher name (WIP)
- Add courses to a personal calendar (WIP)

## Technologies Used

- Rust: A systems programming language known for its speed, reliability, and memory safety.
- Elixir: A functional programming language that runs on the Erlang virtual machine.
- Phoenix: A web framework for Elixir that makes it easy to build high-performance, real-time applications.

## Getting Started

To run this application locally, you will need to have Rust, Elixir, and Phoenix installed on your machine.

1. Clone this repository: `git clone https://github.com/your-username/esgi-calendar-app.git`
2. Navigate to the backend directory: `cd esgi-calendar-app/backend`
3. Install dependencies: `cargo install`
4. Start the backend server: `cargo run`
5. Navigate to the frontend directory: `cd ../frontend`
6. Install dependencies: `mix deps.get`
7. Start the Phoenix server: `mix phx.server`

The application should now be running at `http://localhost:4000`.
You might need to setup some environment variables, please refer to the env.example files if any.

## Contributions

Contributions to this project are welcome. To contribute, please follow these steps:

1. Fork this repository
2. Create a new branch for your feature or bug fix
3. Make your changes
4. Submit a pull request

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Side Note

This project is mainly a way for me to practice the Rust programming language and have some fun with Elixir.
You might see some things that may seem overkill or not well written, if so please take in consideration the previous statement.
And if you want to give me advices on how I can improve the code quality or functionalities, I would love to hear'em out !!

Drop a star If this project is somehow interesting to you, that'll help me staying motivated to ship it :P

My discord: adia#3344
