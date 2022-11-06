
export function Tracker () {
    this.indexPath = '';
    this.list = [];
}
Tracker.prototype = {
    add: function (item) {
        this.list.push(item);
    },
    setIndex: function (path) {
        this.indexPath = path;
    },
    getPre: function () {
        this.list.pop();
        return this.list[this.list.length - 1];;
    }
}