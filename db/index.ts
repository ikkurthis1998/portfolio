import mongoose,  { model, models } from 'mongoose';
import schema from './schema';
import { TLead, TMessage } from './types';

const mongoConnect = mongoose
	.createConnection(
		"mongodb+srv://mongodb:passmongo@cluster1.8bxv8.mongodb.net/?retryWrites=true&w=majority",
		{
            dbName: "portfolio"
		}
)

// export const mongoConnect = async () => {
// 	try {
// 		await mongoose.connect(
// 			"mongodb+srv://mongodb:passmongo@cluster1.8bxv8.mongodb.net/?retryWrites=true&w=majority",
// 			{
// 				dbName: "portfolio"
// 			}
// 		);
// 		console.log("Connected to MongoDB");
// 	} catch (error) {
// 		console.error(error);
// 	}
// };

export const Lead =
	models.Lead || mongoConnect.model<TLead>("Lead", schema.leadSchema, "Lead");

export const Message =
	models.Message ||
	mongoConnect.model<TMessage>("Message", schema.messageSchema, "Message");

// export const Lead = models.Lead || model<TLead>("Lead", schema.leadSchema, "Lead");

// export const Message = models.Message || model<TMessage>(
// 	"Message",
// 	schema.messageSchema,
// 	"Message"
// );

export const db = {
    Lead,
    Message
};
