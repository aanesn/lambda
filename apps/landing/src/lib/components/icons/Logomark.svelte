<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import type { SVGAttributes } from "svelte/elements"

	let { class: className, ...restProps }: SVGAttributes<SVGElement> = $props()
</script>

<svg class={cx("size-7 text-white", className)} viewBox="0 0 18.910847 24" {...restProps}>
	<g transform="translate(-3.544577,-0.99999998)">
		<path
			fill="currentColor"
			d="m 5.308439,0.99999998 h 5.783133 L 22.455423,25 H 16.730121 L 12.826506,16.35422 9.0963868,25 H 3.544577 l 6.563857,-14.16867 z"
		/>
	</g>
</svg>
