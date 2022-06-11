import { Types } from 'mongoose';
import { TMessage } from './message.type';

export type TLead = {
    _id: Types.ObjectId,
    name: string,
    email: string,
    messages: TMessage[],
    createdAt: Date,
    updatedAt: Date
}
