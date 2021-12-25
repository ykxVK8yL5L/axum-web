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
注册请求
```
curl -X POST -i 'http://127.0.0.1:10099/auth/signup' \
  -H "Content-Type: application/json" \
  --data '{
    "username": "user",
    "email": "user@email.com",
    "password": "4S3cr3tPa55w0rd"
  }'
```

登陆请求
```
 curl -X POST -H 'Content-Type: application/json' -i 'http://127.0.0.1:10099/auth/login'  \
  --data '{"username_or_email":"user",  "password":"4S3cr3tPa55w0rd"}'
```





框架使用:    
https://github.com/tokio-rs/axum     
部分代码参考:   
https://github.com/SakaDream/actix-web-rest-api-with-jwt   
还有一个暂时找不到了 等想起来再补充  
