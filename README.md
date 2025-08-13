* A simple implementation of vector database
* Only Supports "fvecs" format for dataset
* Run: 
```
cargo run {dataset file} {query file} {the number of data} {k for top-k search}
```
* Example: sift1m dataset (http://corpus-texmex.irisa.fr/)
```
cargo run datasets/sift/sift_base.fvecs datasets/sift/sift_query.fvecs 10000 10
```