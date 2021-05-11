# Meme Generator

A configuration based random meme generator as a web service.

# Meme CLI
```bash
$ make install
$ meme-gen https://imgflip.com/s/meme/Ancient-Aliens.jpg ./my-meme.jpg 'FEATURES?|BUGS'
$ meme-gen ./my-meme-template.jpg ./my-meme.jpg 'YOLO'
```

# Start Meme Server
1. Setup config file [./config.yml](config.yml)
2. Start server
```bash
$ make run
```
3. Visit http://localhost:8080 to generate random memes