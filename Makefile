.PHONY: flamegraph

flamegraph_r:
	cargo b -r
	mv flamegraph_release.svg flamegraph_release.svg.old
	flamegraph -o flamegraph_release.svg -- target/release/ray_tracing_3d run -o ray_traced_images/some_spheres_5_3.png -p 5 -b 3
	
flamegraph_d:
	cargo b
	mv flamegraph_debug.svg flamegraph_debug.svg.old
	flamegraph -o flamegraph_debug.svg -- target/debug/ray_tracing_3d run -o ray_traced_images/some_spheres_5_3.png -p 5 -b 3
