import { Schema, Types } from 'mongoose';

export const leadSchema = new Schema({
    name: {
        type: String,
        required: true,
        trim: true
    },
    email: {
        type: String,
        required: true,
        trim: true,
        lowercase: true
    },
    messages: [{
        type: Types.ObjectId,
        ref: 'Message'
    }]
}, { timestamps: true, versionKey: false });

