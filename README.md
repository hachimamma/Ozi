# Ozi Discord Bot

![Ozi Banner](assets/banner.jpg)

Ozi is a multipurpose Discord bot written in Rust using [Poise](https://github.com/serenity-rs/poise) and [Serenity](https://github.com/serenity-rs/serenity), featuring slash commands, weather integration, and more. Simple, extensible, and focused on performance!

---

## Features

- 🟢 **Slash Commands**  
  Clean, modern Discord interaction using slash commands.

- 🌦 **Weather Info**  
  Get real-time weather updates using the OpenWeather API.

- 🏓 **Ping & Uptime**  
  Check bot latency and uptime.

- 🧹 **Bulk Message Purge**  
  Use `/purge` to quickly delete multiple messages from a channel.

- ⚡ **Fast & Lightweight**  
  Built in Rust for efficiency and speed.

- 🛠 **Easy Customization**  
  Add your own commands and features with ease.

---

## Getting Started

### 1. Clone the Repository

```sh
git clone https://github.com/multivariablecalculus/Ozi.git
cd Ozi
```

### 2. Create a Discord Bot Application

- Go to the [Discord Developer Portal](https://discord.com/developers/applications), create an application, and add a bot.
- Copy your bot token.

### 3. Set Up Environment Variables

Create a `.env` file in the root directory:

```env
DISCORD_TOKEN=your_discord_token_here
OPENWEATHER_API_KEY=your_openweather_key_here
```

> **Note:** Never commit your `.env` file! It is ignored by `.gitignore` by default.

### 4. Run the Bot

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed, then run:

```sh
cargo run --release
```

---

## Usage

Invite Ozi to your server using the OAuth2 URL with the following scopes:
- `bot`
- `applications.commands`

Once invited, try out some commands:
- `/ping` – Check Ozi's latency.
- `/weather <city>` – Get the current weather.
- `/purge <amount>` – Bulk delete a specified number of messages in a channel (requires Manage Messages permission).
- `/help` – See all commands.

---

## Contributing

Contributions, issues, and feature requests are welcome!  
Feel free to check [issues page](https://github.com/multivariablecalculus/Ozi/issues).

1. Fork this repository
2. Create your feature branch (`git checkout -b feat/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feat/AmazingFeature`)
5. Open a Pull Request

---

## License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for more information.

---

## Credits

- [Serenity](https://github.com/serenity-rs/serenity)
- [Poise](https://github.com/serenity-rs/poise)
- [OpenWeather API](https://openweathermap.org/api)