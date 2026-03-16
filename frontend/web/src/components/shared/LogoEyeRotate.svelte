<script lang="ts">
	let leftEye = $state<SVGGElement>(),
		leftEyeStatic = $state<SVGCircleElement>(),
		rightEye = $state<SVGGElement>(),
		rightEyeStatic = $state<SVGCircleElement>();

	const updateEyes = (e: MouseEvent) => {
		if (!leftEye || !leftEyeStatic || !rightEye || !rightEyeStatic) return;

		updateEye(e, leftEye, leftEyeStatic);
		updateEye(e, rightEye, rightEyeStatic);
	};

	const updateEye = (e: MouseEvent, eye: SVGElement, eyeStatic: SVGElement) => {
		const rect = eyeStatic.getBoundingClientRect();
		const origin = { x: rect.left + rect.width / 2, y: rect.top + rect.height / 2 };

		const dx = e.clientX - origin.x;
		const dy = e.clientY - origin.y;
		const angle = Math.atan2(dy, dx);

		const translateX = Math.cos(angle) * 1.2;
		const translateY = Math.sin(angle) * 1.2;

		eye.style = `transform: translate(${translateX}px, ${translateY}px)`;
	};
</script>

<svelte:window onmousemove={updateEyes} />

<svg width="100%" height="auto" viewBox="0 0 32 32" xmlns="http://www.w3.org/2000/svg" fill="none">
	<defs>
		<!-- Body gradient: sunset-inspired orange to magenta -->
		<linearGradient id="bodyGrad" x1="4" y1="6" x2="28" y2="30" gradientUnits="userSpaceOnUse">
			<stop offset="0%" stop-color="#FFAD60" />
			<stop offset="50%" stop-color="#FF6B81" />
			<stop offset="100%" stop-color="#C86DD7" />
		</linearGradient>

		<!-- Head gradient: soft peach to pink -->
		<radialGradient id="headGrad" cx="16" cy="11" r="9" gradientUnits="userSpaceOnUse">
			<stop offset="0%" stop-color="#FFE3D1" />
			<stop offset="50%" stop-color="#FF9FAF" />
			<stop offset="100%" stop-color="#FF6B81" />
		</radialGradient>

		<!-- Branch gradient: warm earthy brown -->
		<linearGradient
			id="branchGrad"
			x1="6"
			y1="26"
			x2="26"
			y2="30"
			gradientUnits="userSpaceOnUse"
		>
			<stop offset="0%" stop-color="#7A4A2B" />
			<stop offset="100%" stop-color="#4B2A1D" />
		</linearGradient>

		<filter id="headGlow" x="-50%" y="-50%" width="200%" height="200%">
			<feDropShadow
				dx="0"
				dy="0"
				stdDeviation="1.5"
				flood-color="#FF9FAF"
				flood-opacity="0.5"
			/>
		</filter>
	</defs>

	<!-- Branch -->
	<rect x="6" y="26" width="20" height="3" rx="1.5" fill="url(#branchGrad)" />

	<!-- Body -->
	<ellipse cx="16" cy="20" rx="8" ry="8.5" fill="url(#bodyGrad)" opacity="0.95" />

	<!-- Wings -->
	<ellipse cx="8.5" cy="20" rx="3" ry="6" fill="url(#bodyGrad)" opacity="0.8" />
	<ellipse cx="23.5" cy="20" rx="3" ry="6" fill="url(#bodyGrad)" opacity="0.8" />

	<!-- Feet -->
	<path
		d="M13 26 L12 27.5 M13 26 L14 27.5 M19 26 L18 27.5 M19 26 L20 27.5"
		stroke="#FF9FAF"
		stroke-width="1.4"
		stroke-linecap="round"
	/>

	<!-- Head -->
	<g filter="url(#headGlow)">
		<!-- Horizontal oval head -->
		<ellipse cx="16" cy="11" rx="9" ry="7" fill="url(#headGrad)" />

		<!-- Owl Ear Tufts -->
		<path d="M8 5 C7 3, 10 2, 12 5 L12.5 9 C11 7, 10 6, 8 5Z" fill="url(#headGrad)" />
		<path d="M24 5 C25 3, 22 2, 20 5 L19.5 9 C21 7, 22 6, 24 5Z" fill="url(#headGrad)" />

		<!-- Eyes -->
		<circle bind:this={leftEyeStatic} cx="12.5" cy="11" r="3" fill="#FFFFFF" />
		<circle bind:this={rightEyeStatic} cx="19.5" cy="11" r="3" fill="#FFFFFF" />

		<!-- Left Eye -->
		<g bind:this={leftEye}>
			<circle cx="12.5" cy="11" r="1.6" fill="#1A1A1A" />
			<circle cx="13.3" cy="10.3" r="0.6" fill="#FFFFFF" opacity="0.8" />
		</g>

		<!-- Right Eye -->
		<g bind:this={rightEye}>
			<circle cx="19.5" cy="11" r="1.6" fill="#1A1A1A" />
			<circle cx="20.3" cy="10.3" r="0.6" fill="#FFFFFF" opacity="0.8" />
		</g>

		<!-- Beak / Nose -->
		<path
			d="M16 15.5 C15.2 15.5, 15.2 13.8, 16 13.5 C16.8 13.8, 16.8 15.5, 16 15.5 Z"
			fill="#FF6B81"
		/>
	</g>
</svg>
