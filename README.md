# 该项目只是实验性质，用来测试在线编译rust项目，没有实际应用    
## axum-web
尝试用axum做web开发    
目前功能:  
* 静态文件  
* 模板  
* 打包静态文件至可执行文件  
* 采用diesel做orm 数据库用的sqlite3  
命令行格式   
```
axum-web --host 0.0.0.0  --port 10099 --database axum-web.db
```

