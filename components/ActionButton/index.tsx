import { FC, HTMLAttributes, MouseEventHandler } from "react";

export const ActionButton: FC<{ onClick?: MouseEventHandler, className?: string }> = ({ children, onClick, className }) => {
    return (
		<button
			className={`text-secondary-default rounded text-sm md:text-base bg-dark text-light dark:bg-light dark:text-dark ${className}`}
			onClick={onClick}
		>
			{children}
		</button>
	);
}