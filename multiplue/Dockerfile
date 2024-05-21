# Используем официальный образ Rust в качестве базового
FROM rust:1.63.0 as builder

# Устанавливаем рабочую директорию для сборки Rust проекта
WORKDIR /usr/src/primes_multithreading

# Копируем файлы Cargo.toml и Cargo.lock для кеширования зависимостей
COPY Cargo.toml Cargo.lock ./

# Скачиваем зависимости
RUN cargo fetch

# Копируем исходный код проекта
COPY src ./src
COPY benches ./benches

# Компилируем проект
RUN cargo build --release

# Создаем новый этап для запуска
FROM rust:1.63.0

# Устанавливаем рабочую директорию
WORKDIR /usr/src/primes_multithreading

# Копируем скомпилированные файлы из предыдущего этапа
COPY --from=builder /usr/src/primes_multithreading/target /usr/src/primes_multithreading/target
COPY --from=builder /usr/src/primes_multithreading/src /usr/src/primes_multithreading/src
COPY --from=builder /usr/src/primes_multithreading/benches /usr/src/primes_multithreading/benches
COPY --from=builder /usr/src/primes_multithreading/Cargo.toml /usr/src/primes_multithreading/Cargo.toml

# Копируем Python скрипт и файл зависимостей
COPY benchmark_analysis.py .
COPY requirements.txt .

# Устанавливаем Python и необходимые зависимости
RUN apt-get update && apt-get install -y python3 python3-pip
RUN pip3 install -r requirements.txt

# Устанавливаем команду по умолчанию для запуска бенчмарков и анализа результатов
CMD ["sh", "-c", "cargo bench && python3 benchmark_analysis.py"]
