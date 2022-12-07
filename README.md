# Tums: A Thoughtful Uni Management System

## Building

At first you need to switch the release channel to nightly. The following command lets your PC download and enable nightly edition of Rust.
```
rustup default nightly
```

And then clone this repo as follows:
```
git clone https://github.com/melt-adzuki/Tums.git
```

To start developing, just type:
```
cd Tums
docker-compose up
```

Database should be created automatically and Mongo Express will be available at http://localhost:8081. Note that deleting or adding documents manually causes collection ordering shuffled.
