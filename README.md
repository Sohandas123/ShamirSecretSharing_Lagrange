Choose any secret value and in main() function set your secret. Now you have to choose a prime bigger than your secret value and set it in main(). Also set the number of shares n and threshold value k. Usually n = 2*k -1 or 2k.

Here you don't have to do anything except copying the dependencies from cargo.toml . An example is set here for n = 6, k = 3, secret = 1324, prime = 1631 (>1324).

Another thing you can do here. You can change shares in recover section(line :52) in main(). Any combination of k-shares(distict) you can set among n-shares. 

Don't set duplicate shares in recover section in main().