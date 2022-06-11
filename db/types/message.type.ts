import { Types } from "mongoose";

export type TMessage = {
	_id: Types.ObjectId;
	name: string;
	email: string;
	message: string;
	createdAt: Date;
	updatedAt: Date;
};
