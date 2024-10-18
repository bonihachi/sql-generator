cd ..

cargo build

mkdir ~/sql-generator
Copy-Item ./target/debug/sql-generator.exe ~/sql-generator
Copy-Item -Recurse ./tables ~/sql-generator
