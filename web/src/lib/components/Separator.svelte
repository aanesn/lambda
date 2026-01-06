<script lang="ts">
	import { Separator } from "bits-ui"
	import { cx } from "$lib/utils"

	let { class: className, ...restProps }: Separator.RootProps = $props()
</script>

<Separator.Root
	class={cx(
		"shrink-0 bg-white/15 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px",
		className
	)}
	{...restProps}
/>
