benchmark tool

- h1
    - 支持 -H "Header: Value"
- h2
- h3
- tls handshake：握手后就关闭连接，压测纯握手性能
    - 支持指定 tls ciphersuite
- tcp accept: tcp连接后就关闭，压测tcp accept能力

- 在确定的时间内发送固定数量的http请求并要求server响应，并记录请求失败数-测试server优雅重启


## release

```
git tag v1.0.0
git push --tags
```