import { FC, HTMLAttributes, MouseEventHandler } from "react";

export const ActionButton: FC<{
	onClick?: MouseEventHandler;
	className?: string;
	type?: "button" | "submit" | "reset" | undefined;
}> = ({ children, onClick, className, type }) => {
	return (
		<button
			className={`text-secondary-default rounded text-sm md:text-base bg-dark text-light dark:bg-light dark:text-dark ${className}`}
			onClick={onClick}
			type={type || undefined}
		>
			{children}
		</button>
	);
};