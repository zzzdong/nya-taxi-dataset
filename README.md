nyc-taxi-dataset
=================

Get and load nyc taix yellow trip data into DataBend.


```shell

cd data && wget -c -i urls.txt && cd -

RUSTFLAGS="-C target-cpu=native" cargo run --release

curl -XPUT 'http://root:@127.0.0.1:8000/v1/streaming_load' -H 'insert_sql: insert into nyc_taxi.yellow_trip format CSV' -H 'skip_header: 0' -H 'field_delimiter: ,' -H 'record_delimiter: \n' -F 'upload=@"./output/data.csv"'

```