{ pkgs ? import <nixpkgs> {} }:

pkgs.runCommand "engram-assets" {
  buildInputs = [ pkgs.librsvg ]; # Provides rsvg-convert
} ''
  mkdir -p $out

  # ---------------------------------------------------------------------------
  # 1. Generate the Core Logo SVG
  # ---------------------------------------------------------------------------
  cat > $out/engram-logo.svg <<EOF
<svg width="512" height="512" viewBox="0 0 512 512" fill="none" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="sparkGradient" x1="100" y1="100" x2="400" y2="400" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#FBBF24" /> <stop offset="50%" stop-color="#F97316" /> <stop offset="100%" stop-color="#DC2626" /> </linearGradient>
    
    <filter id="warmGlow" x="-50%" y="-50%" width="200%" height="200%">
      <feGaussianBlur stdDeviation="3.5" result="blur"/>
      <feComposite in="SourceGraphic" in2="blur" operator="over"/>
    </filter>
  </defs>

  <g transform="translate(256, 256)">
    <path d="M0 -65 L0 -125" stroke="url(#sparkGradient)" stroke-width="10" stroke-linecap="round" />
    <path d="M56 32 L108 62" stroke="url(#sparkGradient)" stroke-width="10" stroke-linecap="round" />
    <path d="M-56 32 L-108 62" stroke="url(#sparkGradient)" stroke-width="10" stroke-linecap="round" />

    <circle cx="0" cy="-145" r="22" fill="#FBBF24" />
    <circle cx="125" cy="72" r="22" fill="#DC2626" />
    <circle cx="-125" cy="72" r="22" fill="#F97316" />

    <path d="M0 -60 L52 -30 L52 30 L0 60 L-52 30 L-52 -30 Z" 
          fill="#1c1917" 
          stroke="url(#sparkGradient)" 
          stroke-width="8"
          filter="url(#warmGlow)"/>
          
    <circle cx="0" cy="0" r="14" fill="#FEF3C7"/>
  </g>
</svg>
EOF

  # ---------------------------------------------------------------------------
  # 2. Generate the Social Preview SVG (Layout)
  # ---------------------------------------------------------------------------
  # This constructs a 1280x640 layout with the logo on the left and text on the right.
  # Text uses the gradient fill + drop shadow for dual-mode visibility.
  
  cat > social-layout.svg <<EOF
<svg width="1280" height="640" viewBox="0 0 1280 640" fill="none" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="textGradient" x1="0" y1="0" x2="100%" y2="0" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#FBBF24" />
      <stop offset="50%" stop-color="#F97316" />
      <stop offset="100%" stop-color="#DC2626" />
    </linearGradient>
    
    <filter id="textShadow" x="-20%" y="-20%" width="140%" height="140%">
      <feFlood flood-color="#000000" flood-opacity="0.6"/>
      <feComposite in2="SourceAlpha" operator="in"/>
      <feOffset dx="2" dy="2" result="offsetblur"/>
      <feMerge> 
        <feMergeNode in="offsetblur"/>
        <feMergeNode in="SourceGraphic"/> 
      </feMerge>
    </filter>
  </defs>

  <rect width="1280" height="640" fill="none"/>

  <g transform="translate(360, 320) scale(0.9)">
    $(cat $out/engram-logo.svg | grep -v '<?xml' | grep -v '<svg' | grep -v '</svg>')
  </g>

  <g transform="translate(640, 320)" font-family="sans-serif" font-weight="bold">
    <text x="0" y="10" font-size="120" fill="url(#textGradient)" filter="url(#textShadow)" dominant-baseline="middle">
      Engram
    </text>
    
    <text x="5" y="80" font-size="42" fill="#9CA3AF" filter="url(#textShadow)" dominant-baseline="middle">
      Distributed Memory for AI
    </text>
  </g>
</svg>
EOF

  # ---------------------------------------------------------------------------
  # 3. Convert Social Layout to PNG
  # ---------------------------------------------------------------------------
  # We use rsvg-convert to render the transparent PNG.
  
  rsvg-convert -w 1280 -h 640 social-layout.svg -o $out/engram-social-preview.png
''
