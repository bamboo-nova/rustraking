# Abstruct
Rustからpythonファイルを呼び出してトラッキングを実施する。

# Usage

1. .envのコピー

```
$ cp .env{.example,}
```

2. dockerイメージの構築

```
$ DOCKER_BUILDKIT=1 docker build . -t rust-track-image
```

3. コンテナの起動

```
$ docker compose run --rm rust-track-demo
```

4. 実行

```
$ cargo run -- -m yolov8n.pt <動画ファイル>
```