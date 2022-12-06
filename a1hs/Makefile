
build:
	@stack --verbosity error exec -- ghc app/Main.hs --make -o bin/Main -odir bin -hidir bin

buildi:
	@stack --verbosity error exec -- ghci app/Main.hs --make -o bin/Main -odir bin -hidir bin

fast:
	@stack build --fast --exec app

run: build	
	@bin/Main

