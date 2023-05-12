import {
  Contact,
  Message,
  ScanStatus,
  WechatyBuilder,
  log,
}from 'wechaty'

import axios from 'axios'

// this method not work with import, found this link to solve: https://github.com/gtanner/qrcode-terminal/issues/47
// import { generate } from 'qrcode-terminal'

const qr_code = require('qrcode-terminal');
const ipaddr = 'http://127.0.0.1:3000';

function onScan (qrcode: string, status: ScanStatus) {
    if (status === ScanStatus.Waiting || status === ScanStatus.Timeout) {
        qr_code.generate(qrcode, { small: true })  // show qrcode on console

        const qrcodeImageUrl = [
        'https://wechaty.js.org/qrcode/',
        encodeURIComponent(qrcode),
        ].join('')
            log.info('StarterBot', 'onScan: %s(%s) - %s', ScanStatus[status], status, qrcodeImageUrl)
        } else {
            log.info('StarterBot', 'onScan: %s(%s)', ScanStatus[status], status)
        }
  }

function onLogin (user: Contact) {
    log.info('StarterBot', '%s login', user)
}

function onLogout() {
  log.info('Logout')
}

async function onMessage (msg: Message) {
    log.info('StarterBot', msg.toString())
    let text: String = msg.text()
    let bot_prefix = text.startsWith('@bot')
    if (bot_prefix) {
        let cmds = text.substring(text.indexOf('q'), text.length).split(' ')

        // proactive query for the latest match record of the player with the given account_id
        // @bot q account_id
        if (bot_prefix && cmds[0] === 'q') {
            axios({
                baseURL: ipaddr,
                url: "/match/latest",
                method: "get",
                params: {
                    account_id: cmds[1]
                }
            })
                .then(async resp => {
                    console.log(resp.data)
                    await msg.say(resp.data)
                })
                .catch(e => {
                    log.error(e)
                })

        }


        if (text.endsWith('牛')) {
            await msg.say('牛')
        }
    }
    // any message received
    if (text.match('没办法') != null) {
        await msg.say('装模作样')
    }
}

const bot = WechatyBuilder.build({
     name: 'bot',
     puppetOptions: {
         // uos must be true to enable WeChat login
         uos: true
    },
  })

bot.on('scan',    onScan)
bot.on('login',   onLogin)
bot.on('logout',  onLogout)
bot.on('message', onMessage)

bot.start()
  .then(() => log.info('StarterBot', 'Starter Bot Started.'))
  .catch(e => log.error('StarterBot', e))
