use std::collections::HashMap;

pub fn tr(is_cn: bool, text: &str) -> String {
    if is_cn {
        return text.to_string();
    }

    let mut items: HashMap<&str, &str> = HashMap::new();
    items.insert("出错", "Error");
    items.insert("原因", "Reason");
    items.insert("删除成功", "Delete success");
    items.insert("删除失败", "Delete failed");
    items.insert("添加成功", "Add success");
    items.insert("添加失败", "Add failed");
    items.insert("复制失败", "Copy failed");
    items.insert("复制成功", "Copy success");
    items.insert("清空失败", "Delete failed");
    items.insert("清空成功", "Delete success");
    items.insert("保存失败", "Save failed");
    items.insert("保存成功", "Save success");
    items.insert("重置成功", "Reset success");
    items.insert("刷新成功", "Flush success");
    items.insert("发送失败", "Send failed");
    items.insert("下载成功", "Download success");
    items.insert("下载失败", "Download failed");
    items.insert("加载失败", "Load failed");
    items.insert("密码错误", "Password Invalid");
    items.insert("正在重试...", "Retrying...");
    items.insert("正在下载...", "Downloading...");
    items.insert("创建账户成功", "Create account success");
    items.insert("创建账户失败", "Create account failed");
    items.insert("密码错误", "Wrong password");
    items.insert("修改密码成功", "Change password success");
    items.insert("切换网络成功", "Switch network success");
    items.insert("非法输入", "Invalid input");
    items.insert("生成交易失败", "Generate transaction failed");
    items.insert("发送交易成功", "Send transaction success");
    items.insert("发送交易失败", "Send transaction failed");
    items.insert("非法交易", "Invalid transaction");
    items.insert("写入成功", "Write file success");
    items.insert("取消成功", "Cancel success");
    items.insert("解码成功", "Decode success");
    items.insert("文件名为空", "File name is empty");
    items.insert("非法文件", "Invalid file");
    items.insert("行情", "Latest");
    items.insert("热门", "Trending");
    items.insert("原文链接", "Source Link");
    items.insert("刷新", "Refresh");
    items.insert("正在刷新", "Refreshing");
    items.insert("关于", "About");
    items.insert("在线", "Online");
    items.insert("正忙", "Busy");
    items.insert("空闲", "Idle");
    items.insert("中文", "En");
    items.insert("排名", "Rank");
    items.insert("代币", "Symbol");
    items.insert("价格", "Price");

    items.get(text).unwrap_or(&text).to_string()
}
