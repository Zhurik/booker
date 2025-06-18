# Booker

[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)

Простой бот для бронирования мест в unSpot, со сверкой в [IsDayOff](https://isdayoff.ru)

## Установка

### Ubuntu

```bash
echo "deb [trusted=yes] https://zhurik-deb.ar.cloud.ru main universe" | sudo tee -a /etc/apt/sources.list
sudo apt-get update
sudo apt-get install booker
```

После этого проверяем:

```bash
booker --version
```

## Настройка

1. Для работы требуется получить Bearer токен и идентификатор места из Unspot. Их можно вытащить из консоли в браузере.

2. Далее можно либо выставить переменные окружения

```bash
export UNSPOT_TOKEN=<здесь токен>
export SPOT_ID=<UUID места>
export UNSPOT_URL=<если хост Unspot отличается от https://unspot.com/>
booker
```

либо же передавать как параметры при запуске

```bash
booker \
    --unpot_token=<здесь токен> \
    --spot_id=<UUID места> \
    --unspot_url=<если хост Unspot отличается от https://unspot.com/>
```

## Автоматизация

### CRON

В `/etc/crontab` добавляем:

```text
1  0    * * *   <ваш пользак>  . /home/<ваш пользак>/booker.sh; booker
```

где `/home/<ваш пользак>/booker.sh` -  это файл вида

```bash
export UNSPOT_TOKEN=<здесь токен>
export SPOT_ID=<UUID места>
export UNSPOT_URL=<если хост Unspot отличается от https://unspot.com/>
```

Дальше можно подкручивать параметры или настраивать как Вам удобно

## TODO

- docker

- MacOS

- Инструкция для systemd

- Интеграция с tg для ошибок
