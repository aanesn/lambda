<script lang="ts">
	import { cva, cx } from "$lib/utils"
	import type { VariantProps } from "cva"
	import type { HTMLAnchorAttributes } from "svelte/elements"

	const link = cva({
		base: "text-sm transition-colors",
		variants: {
			intent: {
				primary: "hover:text-white/80",
				secondary: "text-neutral-500 hover:text-white"
			}
		}
	})

	let {
		class: className,
		children,
		intent = "primary",
		...others
	}: HTMLAnchorAttributes & {
		intent?: VariantProps<typeof link>["intent"]
	} = $props()
</script>

<a class={cx(link({ intent }), className)} {...others}>
	{@render children?.()}
</a>
