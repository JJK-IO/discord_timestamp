<a name="readme-top"></a>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

Quick and simple rust project that also solves a problem. Generate a timestamp link for Discord that is timezone aware to help coordinate across timezones.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.
* rust
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/JJK-IO/discord_timestamp.git
   ```
2. Build and run
   ```sh
   cargo run -- 12:30 --date=2/29/2024 --timezone="America/Denver"
   ```
3. Build for release
   ```sh
   cargo build --release
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- USAGE EXAMPLES -->
## Usage

```sh
Utility to convert a timezone aware date time into a discord time tag <t:timestamp>

Usage: discord_timestamp [OPTIONS] <TIME>

Arguments:
  <TIME>  The time you want to get the timestamp for

Options:
  -d, --date <DATE>          Date as MM/DD/YYYY, defaults to today
  -t, --timezone <TIMEZONE>  Timezone as an IANA string... E.G. "America/Denver". Defaults to your local timezone if possible, otherwise ETC/UTC
  -h, --help                 Print help
  -V, --version              Print version
```

```sh
> ./discord_timestamp 2:30pm --date 3/22/2024 --timezone="America/Los_Angeles" 
<t:1711143000>
```
<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- CONTRIBUTING -->
## Contributing

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>
