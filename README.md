# 该项目只是实验性质，用来测试在线编译rust项目，没有实际应用    
## axum-web
尝试用axum做web开发    
介绍视频: https://youtu.be/LPD0rtw6jzg  
Docker: https://hub.docker.com/r/ykxvk8yl5l/axum-web   
目前功能:  
* 静态文件  
* 模板  
* 打包静态文件至可执行文件  
* 采用diesel做orm 数据库用的sqlite3  
* 已具备基础的JWT认证:注册、登陆、退出。采用Token认证  
* 兼容嵌入式设备Openwrt也可用


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

openwrt   
x86_64的安装代码:   
```
wget https://github.com/ykxVK8yL5L/axum-web/releases/download/v0.0.4/axum-web_0.0.4_x86_64.ipk
wget https://github.com/ykxVK8yL5L/axum-web/releases/download/v0.0.4/luci-app-axum-web_1.0.0_all.ipk
wget https://github.com/ykxVK8yL5L/axum-web/releases/download/v0.0.4/luci-i18n-axum-web-zh-cn_1.0.0-1_all.ipk
opkg install axum-web_0.0.4_x86_64.ipk
opkg install luci-app-axum-web_1.0.0_all.ipk
opkg install luci-i18n-axum-web-zh-cn_1.0.0-1_all.ipk
```

其它 CPU 架构的路由器可在 [GitHub Releases](https://github.com/ykxVK8yL5L/axum-web/releases) 页面中查找对应的架构的主程序 ipk 文件下载安装， 常见
OpenWrt 路由器 CPU 架构如下表（欢迎补充）：

|      路由器     |        CPU 架构       |
|----------------|----------------------|
| nanopi r4s     | aarch64_generic      |
| 小米 AX3600     | aarch64_cortex-a53  |
| 斐讯 N1 盒子    | aarch64_cortex-a53   |
| Newifi D2      | mipsel_24kc          |
| Pogoplug       | arm_mpcore           |

> Tips: 不清楚 CPU 架构类型可通过运行 `opkg print-architecture` 命令查询。




框架使用:    
https://github.com/tokio-rs/axum     
部分代码参考:   
https://github.com/SakaDream/actix-web-rest-api-with-jwt   
