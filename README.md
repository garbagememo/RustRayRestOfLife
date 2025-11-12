レイトレ一週間の続編、RestOfLife編をRustで実装してみる。  
基本的には[Rustではじめるレイトレーシング入門](https://github.com/mebiusbox/docs/blob/master/Rust%E3%81%A7%E3%81%AF%E3%81%98%E3%82%81%E3%82%8B%E3%83%AC%E3%82%A4%E3%83%88%E3%83%AC%E3%83%BC%E3%82%B7%E3%83%B3%E3%82%B0%E5%85%A5%E9%96%80.pdf)
に則ってますが、各種細かい所が違ってます  
写経元と違い、BVH実装あり。ローカルで試した限り、軸が長い方向に分割しても乱数とほとんどの場合変わらないのでBVH分割軸はランダム。
![ランダム球表示](https://github.com/garbagememo/RustRayRestOfLife/blob/main/image.png "サンプル画像")
