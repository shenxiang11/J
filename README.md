## J

J 是一个命令行中的 JWT 生成与校验工具。

## 用法

```bash
J  sign --algo RS256 --sub 1 --aud 2 --exp 20s --secret ./fixtures/private_key.pk 
```

```bash
J verify --algo RS256 --secret ./fixtures/public_key.pk -t eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiIxIiwiYXVkIjoiMiIsImV4cCI6MTcyNTI0ODI2N30.Bgb36QInV3y7AqLoKmz5P-EoH56g3Jf9VB95b5upFBy5-ff1NgoEVYUzBQvnST7KExx7wv7o1WQRbhCqaHG4NNqQLGouJLBikxdKBGWZ98wYGiCuxTge1ePgQ3TX4R04u2RqjkJbeWBSX6vSNGRiQhBlmkwfiJGui4coh0XBDYUQqsJer570tNreY_8-RGDJU5-TIlQ4ZmsX-BzAvwjXWVrCSGGfkIPZZETBtgEjsDqZjapp2t8zqgx_soT2TMpEH85yQklREwMPYi5M_90EMB5Lbobz-wJHDopwAeGWNuhhRhtd2zB_nxe4EEvCEbGEDlF4T_7c8SQ1__HE19pUuOc9PPaVb75zAia71fLrFwulywd76t5b-m0DPxo_RFvBn5fte9xbd4V03xf4DDRacSCVWFCo3xmZSB_7J5EnL42NnetIej4j6SavtUb0kDW5xGRfS2NzuVky0K56hUzdZAu6mKobktfsxJmFThY8gCnak9TDP0Y_vmyGkPzWyeAX-ogwo-n8_S4Vrgb9txLgSpo_l6-B4yoG2yqQJB-S94YUO7A5MCH5WGWy2BJLKTjuYI1gm801LmpTYzhG6zWVUHvRN3krf2Nk8eh2loM8WXz1KQ0Jkno7Qa9pv3azDTq_UwOfiPNZ8_hJ4VTrU0dPSnJeywafPc_TMLcZdj613Go
```


## 截图

生成 JWT:

![](./pics/1.png)


校验 JWT 成功:

![](./pics/2.png)


JWT 过期导致校验失败:

![](./pics/3.png)


生成的 JWT 通过 https://jwt.io/ 的校验：

![](./pics/4.png)
