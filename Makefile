.PHONY: clean
clean:
	@echo "Cleaning up..."
	cd chap1 && cargo clean
	cd chap2 && cargo clean
	cd chap3 && cargo clean
	cd chap4 && cargo clean