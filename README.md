# rust 写的一个尔罗斯方块游戏

- 项目地址：[https://github.com/Topgt/rust_block_game](https://github.com/Topgt/rust_block_game)  
- 项目介绍：使用rust语言编写的游戏，使用react框架进行前端渲染，使用wasm-pack进行rust编译成wasm文件，使用npm link命令连接rust项目和react项目。
- 使用npm link 命令连接安装 block_game [(react项目)](https://github.com/Topgt/react_block_game)

- 项目运行：
```
git clone https://github.com/Topgt/rust_block_game.git
cd ./rust_block_game
wasm-pack build
cd ./pkg
npm link

cd ../..

git clone https://github.com/Topgt/react_block_game.git
cd ./react_block_game
npm link block_game --save-dev
npm install 

npm start

```