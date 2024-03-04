.PHONY: node application

genesis:
	cometbft init

clear:
	rm -r -f ~/.cometbft/

node: genesis 
	cometbft node

application:
	cd application && cargo run


