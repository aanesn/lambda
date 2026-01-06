<script lang="ts">
	import { cx } from "$lib/utils"
	import { DropdownMenu } from "bits-ui"

	let { class: className, ...restProps }: DropdownMenu.ItemProps = $props()
</script>

<DropdownMenu.Item
	class={cx(
		"relative flex cursor-default items-center gap-2 rounded-lg px-2 py-1.5 text-sm outline-hidden select-none data-highlighted:bg-white/10 [&_svg]:pointer-events-none [&_svg]:shrink-0",
		className
	)}
	{...restProps}
/>
