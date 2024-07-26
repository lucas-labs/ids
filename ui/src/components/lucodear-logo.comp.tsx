export interface LogoProps {
    size: number;
}

export function LucodearLogo({ size }: LogoProps) {
    return (
        <svg viewBox='0 0 32 32' height={size}>
            <title>lucodear</title>
            <path
                className='fill-current'
                fill='chartreuse'
                d='M18,6v8H14V6Zm4-4H18V6h4ZM8,20V14h2V12H4v8H2v2H4v8h6V28H8V22H6V20Zm20,0V12H22v2h2v6h2v2H24v6H22v2h6V22h2V20Z'
            />
        </svg>
    );
}
