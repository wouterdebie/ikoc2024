# Chewbacchus 2024 Throw

## Description
This is the code for throws I made for the [Chewbaccus](https://chewbacchus.org) 2024 parade. It's a
Vogon Poetry transceiver that sends and receives poetry from and to other
devices. It's based on an esp32 board and uses an SSd1306 OLED display. This project is written
in Rust and uses the esp-idf framework.

![cbthrow](https://github.com/wouterdebie/ikoc2024/assets/172038/ec69a8aa-b6db-4276-a31a-f31c9ef76d36)

[Demo](https://photos.app.goo.gl/udVBA6jNoyL7UNUZA)

## Building and installing
Installing and running: `DEVICE_ID=1 cargo run --release -- -p /dev/cu.usbserial-1410 -b 256000`.
Make sure to change the device ID to a unique number for each device. Secondly, make sure you have
an esp32 toolchain with `espup` installed.

## Inner workings

Devices have a hardcoded list of 42 poems that they can send and receive. Devices use ESP-NOW to
broadcast "poems" to one another. The protocol is very simple: a device sends a packet with two
bytes; a POEM_ID (0..41) and their DEVICE_ID (1..42). Other devices are listening and display
the sent poem as soon as they receive them.

If a device hasn't received a poem in 10 seconds, it will pick a random poem.

## Hardware
For the project I used an [AITIP ESP32 Lite v1.0.0](https://www.amazon.com/gp/product/B0BCJT8KDX/ref=ppx_yo_dt_b_search_asin_title?ie=UTF8&th=1) and a [Makerfocus SSD1306 OLED Display](https://www.amazon.com/gp/product/B08LQM9PQQ/ref=ppx_yo_dt_b_search_asin_title?ie=UTF8&psc=1). I connected pin 0 to SDA and pin 4 to SCL.

## Thanks

- Intergalactic Krewe of Chewbacchus for organizing the parade. And their Overlords for awarding me with the best throw award.
- r/vogonpoetrycircle the poems that aren't written by Douglas Adams.

```
                                                   p
            p                                 aaQW'
           j'                             qaQQQQP
           Q,      qaayQQQbaa,       q  a?4QQQQ'
           ]bp   aQW@???????QQQaaQQaayQQpa ?QP'
            "QQQP?'          )WQQbaaj?Q ??py"
                             jQQ)??? yQp\aP?4ap
               ap            ]Qf    qQQQQP   )4ba  .aa
            w???4QQbag      qmQf    jQQQ'       ?$amQQQQp
           ?      )?4QQQaaayQQP     mQP'        ]QQQQQQQQf
            aaap      ]??????'      Q?          )WQQQQQQQQ'
          ]QQQW            wWQQQWbp]'            yQQQQQ@$bp Qf
          ]QQQbaaap       aWWQQQQQQb             ]QQQQf  )?QQ'
        qayQQQQQ??!      jQQQQQQQQQQw           aQQQQQb
        QQQQQQQf         =QQQQQQQQQQQ         _QQQQQQQQp
        ]QQQQQQbmQbp     )4QQQQQQQQQQP    q qawQQQQQQQb
         4QQQQQQP??'     yQQQQQQQQQQP4FQQQQQQQWQQQQQQP?
          4QQQQf         jQQQQQQQQQQfyfQQQQQQQQQQQQQQ
         jQQQQQQa      qaaQQQQQQQQQQwWpQQQQQQQQQQQ@P'
        qQQQQQQQp    aQQQWQQQQQQQQQD?Q?QQQQQ4Q?Y "
        P4QQQQQQQ[qaQWQQQQQQQQQQQQQP4QQQQQQQ,'
         QQQQQQQQyQQWQQQQQQQQQQQQQWtjf]QQQQQL
         ']QQQQQQWQQQQQQQQQQQQQQQQbaQ jQQQQQ!
         _QQQQQQQQQQQQQQQQQQQQQQQQ?4P4QQQQQP'
         ]4QQQQQQQ@WPQQQQQQQQQQQQP4@$QQQQQ@?p
          j??4QQQP ? |QQQQQQQQQQP m'jQQQQQ'
             ]' ?    :@ QQQQQQQQwybqQQQQQgp qawQaaaap
                        QQ4QQQQfj?4QQQQQQQQQQQWQQWWQyQQbap
                        ]P PQQQWWQQQQQQQQQQQQQQQQQQQQQQQQQQbp
                            yQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQba
                         qamQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQa
                        q?QQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQwp
                        qmQQQQQQQQQQQQQ???)QQ4QQQQQQQQQQQQQQQQQQQ[
                       yQQQQQQQQQQQQQ?     )[ 4[4QP??QQQQQQQQQQQ4f
                      ]wQQQQQQQQQQQQk            "aQQQQQQQQQQQQQ
                     .yQQQQQQQQQQQQ4f             aQQQQQQQQQQQPW
                     ]QQQQQQQQQQQQW)             )ajQQQQQQQQQP`
                    q4QQQQQQQQQQQ@f              qjQQQQQQQ'J!
                    qyQQQQQQQQQQQf'          qyQQQQQQQQP'"
                   )WQQQQQQQQQQQ4`          ]QQQQQQQQT'
                   ]QQQQQQQQQQ )            ]QQQQQQ?
                   j?QQQQQQQQQQa             ]QQQQf
                   " QQQQQQQQQQQ',           jQQQQf
                    j4QQQQQQQQQQ             4QQQQQQp
                     ]]QQQQQQQWF              ?$QQQQP
                     qQQQQQQQ4[                WQQQP'
                     m?QQQQ@ )                  ??
                     '_QQQQQ
                    _aQQQQQQb
                  gyQQQQQQQP^
                 yQQQQQQQQf
                  mQQQQQQQ
                   )WQQQQW
                     "?"?'
```
