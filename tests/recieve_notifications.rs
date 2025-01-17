use futures::TryStreamExt;
use odilia_notify::listen_to_dbus_notifications;
use std::{error::Error, time::Duration};

use notify_rust::Notification;

#[tokio::test]
async fn test_listen_to_dbus_notifications() -> Result<(), Box<dyn Error>> {
    // Spawn a new task to listen for notifications
    let listener_task = tokio::spawn(async move {
        let mut stream = listen_to_dbus_notifications().await;
        //we're only interested in the first notification from the stream
        //race conditions: if another notification happens before this one, for example on a real freedesktop powered linux system, that one will be picked up by this test, causing it to fail
        let notification = stream.try_next().await.unwrap().unwrap();
        assert_eq!(notification.app_name, "test-notify");
        assert_eq!(notification.body, "test body");
        assert_eq!(notification.title, "test summary");
        assert!(notification.actions.is_empty());
        Ok::<(), Box<dyn Error + Send>>(())
    });
    // Delay sending the notification
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Send a Notification to see if it's correctly recieved on the other side
Notification::new()
.appname("test-notify")
.summary("test summary")
.body("test body")
.show_async()
.await?;
    // Await the listener task
    listener_task.await?.unwrap();
    Ok(())
}
