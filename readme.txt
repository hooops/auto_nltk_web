不用抓数据，不同的url 可以对应不同的网页内容，
比如  http：xxx/html/efefe3fde3fd
http：xxx/html/efefe3fde3fdfewfewfewfewf
http：xxx/html/sdfdsfs
html/ 后面的可以自定义 生产的网页前端的内容符合阅读语法，搜索引擎可以正确解析，前端ui可以自己换，默认的有点简陋
也就是： url 后面有10000000000000或者无限个网页 
测试并发： 1核 1g内存的服务器 每秒400个左右百分百成功im，也就是，一天支持1000万访问量，rust语言

 ab -n 1000 -c 100 -t 10 http://xxxx/html/dcfcd07e645d245babe887e5e2daa016dcfcd07e645d245babe887e5e2daa016dcfcd07e645d245babe887e5e2daa016.html


 Benchmarking en0.figureaspect.com (be patient)
Finished 4549 requests


Server Software:
Server Hostname:        en0.figureaspect.com
Server Port:            80

Document Path:          /html/dcfcd07e645d245babe887e5e2daa016dcfcd07e645d245babe887e5e2daa016dcfcd07e645d245babe887e5e2daa016.html
Document Length:        28894 bytes

Concurrency Level:      100
Time taken for tests:   10.001 seconds
Complete requests:      4549
Failed requests:        0
Write errors:           0
Total transferred:      132429982 bytes
HTML transferred:       131885081 bytes
Requests per second:    454.85 [#/sec] (mean)
Time per request:       219.851 [ms] (mean)
Time per request:       2.199 [ms] (mean, across all concurrent requests)
Transfer rate:          12931.31 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        2   33 159.0     11    2385
Processing:     5  174 199.9    159    3705
Waiting:        3  130 141.6    142    2705
Total:          7  207 262.2    173    5066

Percentage of the requests served within a certain time (ms)
  50%    173
  66%    223
  75%    238
  80%    252
  90%    378
  95%    648
  98%   1036
  99%   1235
 100%   5066 (longest request)